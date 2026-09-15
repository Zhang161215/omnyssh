//! Docker service provider.
//!
//! Detects Docker and extracts running-container metrics, including the ports
//! each container occupies.

use async_trait::async_trait;

use super::{metric_int, ServiceProvider};
use crate::event::{DockerContainer, ServiceKind};
use crate::ssh::probe::ProbeOutput;

/// Docker service provider.
pub struct DockerProvider;

#[async_trait]
impl ServiceProvider for DockerProvider {
    fn kind(&self) -> ServiceKind {
        ServiceKind::Docker
    }

    fn detect(&self, probe_output: &ProbeOutput) -> bool {
        // Check if Docker section has content
        probe_output.has_section("DOCKER")
    }

    /// Extract basic metrics from Quick Scan docker ps output.
    /// This allows us to show container count immediately.
    fn quick_metrics(&self, probe_output: &ProbeOutput) -> Vec<super::ServiceMetric> {
        let mut metrics = Vec::new();

        if let Some(docker_output) = probe_output.get_section("DOCKER") {
            let containers = parse_containers(docker_output);
            let total = containers.len() as i64;
            let running = containers.iter().filter(|c| is_running(&c.status)).count() as i64;

            metrics.push(metric_int("containers_total", total));
            metrics.push(metric_int("containers_running", running));
        }

        metrics
    }
}

/// Parse `docker ps` rows: `ID\tNames\tStatus\tImage[\tPorts]`.
pub fn parse_containers(docker_output: &str) -> Vec<DockerContainer> {
    docker_output
        .lines()
        .filter_map(parse_container_line)
        .collect()
}

fn parse_container_line(line: &str) -> Option<DockerContainer> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 4 {
        return None;
    }
    Some(DockerContainer {
        id: parts[0].trim().to_string(),
        name: parts[1].trim().to_string(),
        status: parts[2].trim().to_string(),
        image: parts[3].trim().to_string(),
        ports: parts
            .get(4)
            .map(|s| s.trim().to_string())
            .unwrap_or_default(),
    })
}

fn is_running(status: &str) -> bool {
    status.contains("Up")
}

/// Compact host-side port list for UI display.
///
/// Docker's Ports field often duplicates IPv4 and IPv6 (`0.0.0.0:8080->80/tcp,
/// [::]:8080->80/tcp`). Prefer published IPv4 mappings; fall back to the raw
/// string when nothing published is present.
pub fn display_ports(ports: &str) -> String {
    let ports = ports.trim();
    if ports.is_empty() {
        return String::new();
    }

    let published: Vec<&str> = ports
        .split(',')
        .map(str::trim)
        .filter(|part| part.contains("->") && !part.starts_with('['))
        .collect();

    if published.is_empty() {
        ports.to_string()
    } else {
        published.join(", ")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_detect_from_probe() {
        let probe_output = "===OMNYSSH:DOCKER===\nabc123\tnginx\tUp 2 hours\tnginx:latest\n";
        let parsed = ProbeOutput::parse(probe_output).expect("should parse");
        let provider = DockerProvider;
        assert!(provider.detect(&parsed));
    }

    #[test]
    fn test_docker_not_detected_when_absent() {
        let probe_output = "===OMNYSSH:OS===\nUbuntu\n";
        let parsed = ProbeOutput::parse(probe_output).expect("should parse");
        let provider = DockerProvider;
        assert!(!provider.detect(&parsed));
    }

    #[test]
    fn parse_containers_reads_names_and_ports() {
        let output = "\
abc123\tnginx-proxy\tUp 2 hours\tnginx:latest\t0.0.0.0:80->80/tcp, [::]:80->80/tcp
def456\tdb-master\tUp 5 days\tpostgres:15\t0.0.0.0:5432->5432/tcp
ghi789\tworker\tUp 1 minute\tapp:1\t";
        let containers = parse_containers(output);
        assert_eq!(containers.len(), 3);
        assert_eq!(containers[0].name, "nginx-proxy");
        assert_eq!(containers[0].ports, "0.0.0.0:80->80/tcp, [::]:80->80/tcp");
        assert_eq!(containers[1].name, "db-master");
        assert_eq!(containers[1].ports, "0.0.0.0:5432->5432/tcp");
        assert_eq!(containers[2].name, "worker");
        assert!(containers[2].ports.is_empty());
    }

    #[test]
    fn parse_containers_accepts_legacy_rows_without_ports() {
        let output = "abc123\tnginx\tUp 2 hours\tnginx:latest\n";
        let containers = parse_containers(output);
        assert_eq!(containers.len(), 1);
        assert!(containers[0].ports.is_empty());
    }

    #[test]
    fn display_ports_prefers_ipv4_published_mappings() {
        assert_eq!(
            display_ports("0.0.0.0:8080->80/tcp, [::]:8080->80/tcp"),
            "0.0.0.0:8080->80/tcp"
        );
        assert_eq!(display_ports(""), "");
        assert_eq!(display_ports("80/tcp"), "80/tcp");
    }

    #[test]
    fn quick_metrics_count_running_rows() {
        let probe_output = "===OMNYSSH:DOCKER===\n\
abc\tnginx\tUp 2 hours\tnginx:latest\t0.0.0.0:80->80/tcp\n\
def\told\tExited (0) 3 days ago\tbusybox\t\n";
        let parsed = ProbeOutput::parse(probe_output).expect("should parse");
        let metrics = DockerProvider.quick_metrics(&parsed);
        let value = |name: &str| {
            metrics
                .iter()
                .find(|m| m.name == name)
                .map(|m| match m.value {
                    crate::event::MetricValue::Integer(n) => n,
                })
        };
        assert_eq!(value("containers_total"), Some(2));
        assert_eq!(value("containers_running"), Some(1));
    }
}
