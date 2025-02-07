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
                sshagent(credentials: ['ssh-server-credentials']) {
                    sh """
                    ssh -i /var/jenkins_home/.ssh/angel $SERVER_USER@$SERVER_IP <<EOF
                    # Pull the latest Docker image
                    docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                    # Stop the running container if exists
                    docker stop $DOCKER_IMAGE || true
                    # Remove the stopped container
                    docker rm $DOCKER_IMAGE || true
                    # Run the container in detached mode with port forwarding
                    docker run -d -p 8000:8000 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                    EOF
                    """
                }
            }
        }
    }
}
