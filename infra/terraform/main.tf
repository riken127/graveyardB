terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }

  backend "local" {
    path = "terraform.tfstate"
  }
}

provider "aws" {
  region = var.aws_region
}

# Example resource placeholder
resource "null_resource" "cluster" {
  provisioner "local-exec" {
    command = "echo 'Placeholder for cluster provisioning'"
  }
}
