pipeline {
    agent any

    environment {
        IMAGE_NAME = "167.71.164.51:8082/api_pedidos"
        DOCKER_REGISTRY = "167.71.164.51:8082"
        SERVER_IP = "167.71.164.51"
        SSH_CREDENTIALS = "ssh-server-credentials"  // ID de las credenciales SSH en Jenkins
    }

    stages {
        stage('Checkout Code') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                cleanWs() // Limpia el workspace antes del checkout
                git branch: 'develop', url: 'https://github.com/Anglity/api_pedidos.git'
            }
        }

        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                sh "docker build -t ${IMAGE_NAME}:latest ."
            }
        }

        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                withCredentials([usernamePassword(credentialsId: 'nexus-cred', usernameVariable: 'NEXUS_USER', passwordVariable: 'NEXUS_PASS')]) {
                    sh "echo $NEXUS_PASS | docker login -u $NEXUS_USER --password-stdin ${DOCKER_REGISTRY}"
                }
            }
        }

        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                sh "docker push ${IMAGE_NAME}:latest"
            }
        }

        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                sshagent([SSH_CREDENTIALS]) {
                    sh """
                    ssh -o StrictHostKeyChecking=no root@${SERVER_IP} << 'EOF'
                    echo "📥 Pulling la última imagen de Docker..."
                    docker pull ${IMAGE_NAME}:latest

                    echo "🛑 Deteniendo el contenedor existente (si existe)..."
                    docker stop api_pedidos || true

                    echo "🗑️ Eliminando contenedor antiguo (si existe)..."
                    docker rm api_pedidos || true

                    echo "🏃‍♂️ Iniciando nuevo contenedor..."
                    docker run -d --restart unless-stopped --name api_pedidos -p 8000:8000 ${IMAGE_NAME}:latest

                    echo "✅ Despliegue completado exitosamente!"
                    EOF
                    """
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
