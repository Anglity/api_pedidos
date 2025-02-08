pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_nueva"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
        SSH_PASSPHRASE = "Angel2610" // Passphrase de la clave privada
        CONTAINER_NAME = "naughty_rhodes" // Nombre del contenedor en ejecución
    }
    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'develop', url: 'https://github.com/Anglity/api_nueva3.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                sh "echo '$SSH_PASSPHRASE' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    sshagent(credentials: ['ssh-server-credentials']) {
                        sh """
                        ssh -o StrictHostKeyChecking=no -i /var/jenkins_home/.ssh/id_rsa $SERVER_USER@$SERVER_IP << 'ENDSSH'
                        echo "📥 Pulling la última imagen de Docker..."
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

                        echo "🔍 Verificando si el contenedor en ejecución ($CONTAINER_NAME) está corriendo..."
                        if [ \$(docker ps -q -f name=$CONTAINER_NAME) ]; then
                            echo "🛑 Deteniendo el contenedor en ejecución..."
                            docker stop $CONTAINER_NAME
                        fi

                        echo "🗑️ Eliminando contenedor antiguo si existe..."
                        docker rm -f $CONTAINER_NAME || true

                        echo "🔍 Verificando si el puerto 8080 está en uso..."
                        if lsof -i :8080; then
                            echo "⚠️ ERROR: El puerto 8080 está en uso. Liberándolo..."
                            fuser -k 8080/tcp
                        fi

                        echo "🏃‍♂️ Iniciando nuevo contenedor..."
                        docker run -d --restart unless-stopped --name $CONTAINER_NAME -p 8080:8080 $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

                        echo "✅ Despliegue completado exitosamente!"
                        exit
                        ENDSSH
                        """
                    }
                }
            }
        }
    }
    post {
        success {
            echo "🎉 Pipeline completado exitosamente!"
        }
        failure {
            echo "🚨 ERROR: Algo falló en el pipeline, revisa los logs!"
        }
    }
}
