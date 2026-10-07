# Repository setup for `xlee7s/relay`

This source tree is configured for:

- Repository: `https://github.com/xlee7s/relay`
- Panel image: `ghcr.io/xlee7s/relay-panel`
- Node image: `ghcr.io/xlee7s/relay-node`
- Docker Compose project name: `relay`
- Server install directory: `/opt/relay`

## 1. Create and push the repository

Create an empty public repository named `relay` under the `xlee7s` account. Do
not add a README, license, or `.gitignore` on GitHub because those files already
exist here. Then run these commands from this project directory:

```bash
git init
git add .
git commit -m "Initialize relay repository"
git branch -M master
git remote add origin https://github.com/xlee7s/relay.git
git push -u origin main
```

## 2. Allow release workflows

In the new repository, open **Settings → Actions → General → Workflow
permissions**, select **Read and write permissions**, and save. The release
workflows use the repository `GITHUB_TOKEN`; no personal access token is needed.

## 3. Publish the initial Docker images

The one-line installer uses pre-built images by default. Publish the versions
already declared by this source tree:

```bash
git tag v1.2.12
git push origin v1.2.12

git tag node-v1.2.6
git push origin node-v1.2.6
```

The tags trigger these workflows:

- `.github/workflows/docker-release.yml` builds and publishes the multi-arch
  panel image and creates the `v1.2.12` release.
- `.github/workflows/node-release.yml` builds the node binaries and image and
  creates the `node-v1.2.6` release.

Wait for both workflows to finish successfully. In the account package
settings, change `relay-panel` and `relay-node` to **Public** so servers can pull
them without a GHCR login.

## 4. Verify before announcing the installer

```bash
docker pull ghcr.io/xlee7s/relay-panel:1.2.12
docker pull ghcr.io/xlee7s/relay-node:1.2.6
```

After both pulls succeed anonymously, the public installer is ready:

```bash
curl -fsSL https://raw.githubusercontent.com/xlee7s/relay/master/install.sh | bash
```

For a temporary source build before the packages exist, clone the repository
and run:

```bash
RELAYPANEL_BUILD_LOCAL=1 ./deploy.sh
```
