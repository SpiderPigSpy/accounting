pipeline {
    agent any
    stages {
        stage('Build') { 
            steps {
                sh 'cargo build' 
            }
        }
        stage('Test') { 
            steps {
                sh 'cargo test' 
            }
        }
        stage('Release build') { 
            steps {
                sh 'alias rust-musl-builder=\'docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder\'' 
                sh 'rust-musl-builder cargo build --release'
            }
        }
        stage('Create docker image') {
            steps {
                sh 'cp target/x86_64-unknown-linux-musl/release/accounting accounting'
                sh 'docker build -t accounting .'
            }
        }
        stage('Run application') {
            steps {
                sh 'docker run'
            }
        }
    }
}
