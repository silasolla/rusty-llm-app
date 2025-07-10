resource "google_artifact_registry_repository" "main" {
  provider      = google
  location      = var.region
  repository_id = var.repo_name
  description   = "OCR repository for Cloud Run"
  format        = "DOCKER"
}
