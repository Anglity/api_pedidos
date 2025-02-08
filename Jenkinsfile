pipeline {
    agent any

    environment {
        IMAGE_NAME = "167.71.164.51:8082/api_pedidos"
        DOCKER_REGISTRY = "167.71.164.51:8082"
        SERVER_IP = "167.71.164.51"
        SSH_KEY = "server-ssh-key"
    }

    stages {
        stage('Checkout Code') {
            steps {
                git branch: 'develop', url: 'https://github.com/Anglity/api_pedidos.git'
            }
        }

        stage('Build Docker Image') {
            steps {
                sh "docker build -t ${IMAGE_NAME}:latest ."
            }
        }

        stage('Push Image to Nexus') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'nexus-cred', usernameVariable: 'NEXUS_USER', passwordVariable: 'NEXUS_PASS')]) {
                    sh "docker login -u $NEXUS_USER -p $NEXUS_PASS ${DOCKER_REGISTRY}"
                    sh "docker tag ${IMAGE_NAME}:latest ${DOCKER_REGISTRY}/api_pedidos:latest"
                    sh "docker push ${DOCKER_REGISTRY}/api_pedidos:latest"
                }
            }
        }

        stage('Deploy to Server') {
            steps {
                sshagent([SSH_KEY]) {
                    sh "ssh root@${SERVER_IP} 'docker pull ${DOCKER_REGISTRY}/api_pedidos:latest && docker stop api_pedidos || true && docker rm api_pedidos || true && docker run -d --name api_pedidos -p 8000:8000 ${DOCKER_REGISTRY}/api_pedidos:latest'"
                }
            }
        }
    }
}
