variable "aws_region" {
  description = "AWS Region to deploy to"
  type        = string
  default     = "us-east-1"
}

variable "instance_type" {
  description = "EC2 instance type for database nodes"
  type        = string
  default     = "t3.medium"
}

variable "cluster_size" {
  description = "Number of nodes in the cluster"
  type        = number
  default     = 3
}
