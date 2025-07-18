{
  ...
}:

{
  services.alloy.enable = true;

  environment.etc."alloy/config.alloy".text = ''
    prometheus.exporter.unix "default" { }

    prometheus.scrape "node_exporter" {
      targets = prometheus.exporter.unix.default.targets
      forward_to = [prometheus.remote_write.default.receiver]
    }

    // Configure a prometheus.scrape component to collect Alloy metrics.
    prometheus.exporter.self "default" {}
    prometheus.scrape "self" {
      targets    = prometheus.exporter.self.default.targets
      forward_to = [prometheus.remote_write.default.receiver]
    }

    prometheus.remote_write "default" {
      endpoint {
        url = "http://metrics.wip.bar:8428/api/v1/write"
      }
    }

    loki.relabel "journal" {
      forward_to = []
      rule {
        source_labels = ["__journal__systemd_unit"]
        target_label = "systemd_unit"
      }
      rule {
        source_labels = ["__journal__hostname"]
        target_label = "nodename"
      }
      rule {
        source_labels = ["__journal_syslog_identifier"]
        target_label = "syslog_identifier"
      }
    }

    loki.source.journal "journal" {
      forward_to = [loki.write.default.receiver]
      max_age = "12h"

      labels = {job = "systemd-journal"}
      relabel_rules = loki.relabel.journal.rules
    }

    loki.write "default" {
      endpoint {
        url = "http://metrics.wip.bar:3100/loki/api/v1/push"
      }
    }
  '';
}
