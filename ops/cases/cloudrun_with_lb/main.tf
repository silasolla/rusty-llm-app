provider "google" {
  project = var.project_id
  region  = var.default_region
  impersonate_service_account = var.impersonate_service_account
}

module "network" {
  source   = "../../modules/network"
  app_name = var.app_name
  region   = var.default_region
}

module "artifact_registry" {
  source    = "../../modules/artifact_registry"
  region    = var.default_region
  repo_name = var.repo_name
}

module "cloud_run" {
  source     = "../../modules/cloud_run"
  name       = var.app_name
  project_id = var.project_id
  region     = var.default_region
  repo_name  = var.repo_name
  image_name = var.image_name
  roles      = [
    "roles/artifactregistry.reader",
    "roles/logging.logWriter",
    "roles/aiplatform.user",
  ]
  add_public_invoker = var.add_public_invoker
  network    = module.network.network_name
  subnetwork = module.network.subnetwork_name
  deletion_protection = var.is_fixed
}

module "lb" {
  source       = "../../modules/load_balancer"
  app_name     = var.app_name
  region       = var.default_region
  lb_allow_ips = var.lb_allow_ips
  cloud_run_service_name = module.cloud_run.service_name
}
