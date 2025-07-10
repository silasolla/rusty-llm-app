variable "project_id" {
  description = "project id"
  type        = string
}

variable "impersonate_service_account" {
  description = "terraform runner"
  type        = string
}

variable "default_region" {
  description = "The default region for resources"
  default     = "asia-northeast1"
}

variable "lb_allow_ips" {
  description = "lb_allow_ips"
  type        = list
  default     = ["0.0.0.0/0"]
}

variable "app_name" {
  description = "app_name"
  type        = string
  default     = "my-app"
}

variable "repo_name" {
  description = "repo_name"
  type        = string
  default     = "cloud-run-repo"
}

variable "image_name" {
  description = "image_name"
  type        = string
  default     = "my-llm-app"
}

variable "add_public_invoker" {
  description = "add_public_invoker"
  type        = bool
  default     = true
}

variable "is_fixed" {
  description = "is_fixed"
  type        = bool
  default     = false
}
