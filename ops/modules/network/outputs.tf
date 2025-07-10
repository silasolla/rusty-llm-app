output "network_self_link" {
  description = "Self link of the VPC network"
  value       = google_compute_network.main.self_link
}

output "subnetwork_self_link" {
  description = "Self link of the subnetwork"
  value       = google_compute_subnetwork.main.self_link
}

output "network_name" {
  description = "Name of the VPC network"
  value       = google_compute_network.main.name
}

output "subnetwork_name" {
  description = "Name of the subnetwork"
  value       = google_compute_subnetwork.main.name
}
