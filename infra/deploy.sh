set -e

openssl aes-256-cbc -K $encrypted_3d130dceec06_key -iv $encrypted_3d130dceec06_iv -in compute-engine-creds.json.enc -out compute-engine-creds.json -d
curl https://sdk.cloud.google.com | CLOUDSDK_INSTALL_DIR=$HOME CLOUDSDK_CORE_DISABLE_PROMPTS=1 bash
mkdir -p lib
gcloud auth activate-service-account --key-file compute-engine-creds.json
gcloud compute --project "todo-tree-216306" scp todo-tree.tar chbaker0@todo-tree-backend-1:~ --zone "us-west1-b"
gcloud compute --project "todo-tree-216306" ssh chbaker0@todo-tree-backend-1 --zone "us-west1-b" -- "docker kill todo_tree_container; docker rm todo_tree_container; docker load -i todo-tree.tar; docker run -d -p 8080:8080 todo-tree"
