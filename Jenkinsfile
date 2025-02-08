pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_pedidos"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
        SSH_PASSPHRASE = "Angel2610" // Passphrase de la clave privada
    }
    stages {
        stage('Checkout') {
            steps {
                git branch: 'develop', url: 'https://github.com/Anglity/api_pedidos.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                sh "echo '$SSH_PASSPHRASE' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
       stage('Deploy to Server') {
    steps {
        script {
            sshagent(credentials: ['ssh-server-credentials']) {
                // Crear un archivo temporal con los comandos a ejecutar
                sh """
                cat << 'EOS' > deploy_script.sh
                #!/bin/bash
                docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                docker stop $DOCKER_IMAGE || true
                docker rm $DOCKER_IMAGE || true
                docker run -d -p 8000:8000 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                EOS
                """

                // Copiar el archivo al servidor remoto y ejecutarlo
                sh """
                scp -o StrictHostKeyChecking=no -i /var/jenkins_home/.ssh/id_rsa deploy_script.sh $SERVER_USER@$SERVER_IP:/tmp/deploy_script.sh
                ssh -o StrictHostKeyChecking=no -i /var/jenkins_home/.ssh/id_rsa $SERVER_USER@$SERVER_IP "chmod +x /tmp/deploy_script.sh && /tmp/deploy_script.sh"
                """
            }
        }
    }
}
  }  
}