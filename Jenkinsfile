pipeline {
    agent any

    environment {
        IMAGE_NAME = "167.71.164.51:8082/api_pedidos"
        DOCKER_REGISTRY = "167.71.164.51:8082"
        SERVER_IP = "167.71.164.51"
        SSH_CREDENTIALS = "server-ssh-key"  // ID de las credenciales SSH en Jenkins
    }

    stages {
        stage('Checkout Code') {
            steps {
                echo "Clonando código fuente desde GitHub..."
                git branch: 'develop', url: 'https://github.com/Anglity/api_pedidos.git'
            }
        }

        stage('Build Docker Image') {
            steps {
                echo "Construyendo imagen Docker..."
                sh "docker build -t ${IMAGE_NAME}:latest ."
            }
        }

        stage('Push Image to Nexus') {
            steps {
                echo "Subiendo imagen a Nexus..."
                withCredentials([usernamePassword(credentialsId: 'nexus-cred', usernameVariable: 'NEXUS_USER', passwordVariable: 'NEXUS_PASS')]) {
                    sh """
                        docker login -u $NEXUS_USER -p $NEXUS_PASS ${DOCKER_REGISTRY}
                        docker tag ${IMAGE_NAME}:latest ${DOCKER_REGISTRY}/api_pedidos:latest
                        docker push ${DOCKER_REGISTRY}/api_pedidos:latest
                    """
                }
            }
        }

        stage('Deploy to Server') {
            steps {
                echo "Desplegando aplicación en el servidor..."
                sshagent([SSH_CREDENTIALS]) {
                    sh """
                    ssh -o StrictHostKeyChecking=no root@${SERVER_IP} << 'EOF'
                    echo "Pulling latest Docker image..."
                    docker pull ${DOCKER_REGISTRY}/api_pedidos:latest

                    echo "Stopping existing container (if any)..."
                    docker stop api_pedidos || true

                    echo "Removing old container (if any)..."
                    docker rm api_pedidos || true

                    echo "Running new container..."
                    docker run -d --name api_pedidos -p 8000:8000 ${DOCKER_REGISTRY}/api_pedidos:latest

                    echo "Deployment completed successfully!"
                    EOF
                    """
                }
            }
        }
    }
}
