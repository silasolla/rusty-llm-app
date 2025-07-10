# rusty-llm-app

## ローカル実行

```
cd app
cargo run
```

## インフラ構築

`ops/cases/cloudrun_with_lb/terraform.tfvars` を作成

```
project_id = "{my-project}"
impersonate_service_account = "terraform-runner@{my-project}.iam.gserviceaccount.com"
lb_allow_ips = ["0.0.0.0/0"]
app_name = "{my-app-name}"
image_name = "{my-image-name}"
```

サービスアカウント (`terraform-runner@{my-project}.iam.gserviceaccount.com`) を作成して権限を付与

```
- roles/artifactregistry.admin
- roles/artifactregistry.writer
- roles/cloudbuild.builds.editor
- roles/compute.loadBalancerAdmin
- roles/compute.networkAdmin
- roles/compute.securityAdmin
- roles/iam.serviceAccountAdmin
- roles/iam.serviceAccountUser
- roles/resourcemanager.projectIamAdmin
- roles/run.admin
- roles/storage.objectAdmin
```

自分のユーザアカウントに上記サービスアカウントのトークン作成権限を付与

```
gcloud iam sercice-accounts add-iam-policy-binding \
  terraform-runner@{my-project}.iam.gserviceaccount.com \
  --member="user:me@example.com" \
  --role="roles/iam.serviceAccountTokenCreator
```

インフラの適用 (Artifact Registry にイメージが無いと止まる)

```
cd ops/cases/cloudrun_with_lb
terraform init
terraform apply
```

## デプロイ

```
cd app
docker buildx build --platform linux/amd64 -t {my-region}-docker.pkg.dev/{my-project}/cloud-run-repo/{image-name} --push .

# 初回のみ
cd ops/cases/cloudrun_with_lb
terraform apply

# MODEL_NAME は適宜変更
gcloud run services update {my-app-name} --region {my-region} --update-env-vars PROJECT_ID={my-project-id},MODEL_NAME=gemini-2.0-flash-001
```
