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
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'develop', credentialsId: 'github-credentials', url: 'https://github.com/Anglity/api_pedidos.git'
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
                        
                        echo "📥 Descargando la última imagen de Docker..."
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

                        echo "🔍 Verificando si el contenedor $DOCKER_IMAGE está en ejecución..."
                        if [ \$(docker ps -q -f name=$DOCKER_IMAGE) ]; then
                            echo "🛑 Deteniendo el contenedor en ejecución..."
                            docker stop $DOCKER_IMAGE
                        fi

                        echo "🗑️ Eliminando contenedor antiguo (si existe)..."
                        docker rm -f $DOCKER_IMAGE || true

                        echo "🔍 Verificando si el puerto 8000 está en uso..."
                        if lsof -i :8000 | grep LISTEN; then
                            echo "⚠️ El puerto 8000 está en uso. Liberándolo..."
                            fuser -k 8000/tcp
                            sleep 3
                        fi

                        echo "🏃‍♂️ Iniciando nuevo contenedor..."
                        docker run -d --restart unless-stopped --name $DOCKER_IMAGE -p 8000:8000 $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

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
