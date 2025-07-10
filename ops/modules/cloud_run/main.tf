resource "google_cloud_run_v2_service" "main" {
  name     = var.name
  location = var.region
  deletion_protection = var.deletion_protection
  ingress = "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"

  template {
    service_account = google_service_account.main.email
    containers {
      image = "${var.region}-docker.pkg.dev/${var.project_id}/${var.repo_name}/${var.image_name}"
    }
    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"
      network_interfaces {
        network    = var.network
        subnetwork = var.subnetwork
      }
    }
  }

  lifecycle {
    ignore_changes = [template[0].containers[0].env]
  }
}

resource "google_service_account" "main" {
  account_id = "${var.name}-run-account"
}

resource "google_project_iam_member" "main" {
  for_each = toset(var.roles)

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.main.email}"
}

resource "google_cloud_run_service_iam_member" "public_invoker" {
  count    = var.add_public_invoker ? 1 : 0
  location = google_cloud_run_v2_service.main.location
  service  = google_cloud_run_v2_service.main.name
  project  = var.project_id
  role     = "roles/run.invoker"
  member   = "allUsers"
}
