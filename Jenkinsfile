pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_pedidos"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
    }
    stages {
        stage('Checkout') {
            steps {
                // Checkout from the 'develop' branch
                git branch: 'develop', url: 'https://github.com/Anglity/api_pedidos.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                // Build Docker image
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                // Login to Docker registry (Nexus)
                sh "echo 'Angel2610' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                // Push the built image to Nexus
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
        stage('Deploy to Server') {
            steps {
                script {
                    def remote = [
                        name: 'targetServer',
                        host: SERVER_IP,
                        user: SERVER_USER,
                        identityFile: '/var/jenkins_home/.ssh/id_rsa',
                        allowAnyHosts: true
                    ]
                    sshCommand remote: remote, command: """
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                        docker stop $DOCKER_IMAGE || true
                        docker rm $DOCKER_IMAGE || true
                        docker run -d -p 8000:8000 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                    """
                }
            }
        }
    }
}
