# OWASP Dependency-Check — SCA Tool

Software Composition Analysis (SCA) tool from OWASP. Identifies known vulnerabilities in project dependencies. Supports Java, .NET, Python, Ruby, Node.js, and more.

## How It Works

Dependency-Check inspects project build files and dependency manifests. Fingerprints libraries, looks up against NVD (National Vulnerability Database), and reports CVEs.

**Analysis pipeline:**
1. **Scanner** — Identifies dependencies from build files, archives, and package managers
2. **Evidence collection** — Extracts metadata: CPE, PURL, vendor, product, version
3. **Vulnerability lookup** — Matches evidence against NVD, OSS Index, RetireJS, and custom feeds
4. **Suppression** — Applies suppression rules for false positives
5. **Report generation** — HTML, XML, JSON, CSV, SARIF, JUnit formats

**Supported ecosystems:**

| Language | Build Files / Manifests |
|----------|------------------------|
| Java | pom.xml, build.gradle, WAR/JAR/EAR |
| .NET | packages.config, *.csproj, nupkg |
| Python | requirements.txt, setup.py, Pipfile.lock |
| Ruby | Gemfile.lock |
| Node.js | package-lock.json, yarn.lock |
| PHP | composer.lock |
| C/C++ | (via archive scanning or CPE match) |
| Go | go.sum, Gopkg.lock |
| Rust | Cargo.lock |

## Manual

```bash
# CLI scan
dependency-check --scan /path/to/project \
  --format HTML --out /path/to/reports

# Scan with suppression file
dependency-check --scan /path/to/project \
  --suppression suppressions.xml \
  --out /path/to/reports

# Scan single file
dependency-check --scan pom.xml --format JSON --out ./reports

# NVD API key (faster updates)
dependency-check --scan . --nvdApiKey <key> --format HTML

# Update local database (no scan)
dependency-check --updateonly

# Maven plugin
mvn org.owasp:dependency-check-maven:check

# Gradle plugin
gradle dependencyCheckAnalyze
```

## Build

```bash
git clone https://github.com/dependency-check/DependencyCheck.git
cd DependencyCheck
mvn clean install
# Artifact: cli/target/dependency-check-<version>-release.zip
```

## Install

```bash
# Download CLI
wget https://github.com/dependency-check/DependencyCheck/releases/download/v10.0.3/dependency-check-10.0.3-release.zip
unzip dependency-check-10.0.3-release.zip
cd dependency-check/bin
./dependency-check.sh

# Docker
docker pull owasp/dependency-check
docker run --rm -v "$PWD:/src" -v "$PWD/reports:/reports" \
  owasp/dependency-check \
  --scan /src --format HTML --out /reports

# Homebrew
brew install dependency-check
```

## Package

| Manager | Command |
|---------|---------|
| Binary | GitHub releases |
| Homebrew | `brew install dependency-check` |
| Docker | `docker pull owasp/dependency-check` |
| Maven | `dependency-check-maven` plugin |
| Gradle | `org.owasp.dependencycheck` plugin |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://owasp.org/www-project-dependency-check/ |
| GitHub | https://github.com/dependency-check/DependencyCheck |
| Docs | https://owasp.org/www-project-dependency-check/documentation/ |
| Data feeds | https://github.com/dependency-check/DependencyCheck#datafeeds |
| Suppression | https://github.com/dependency-check/DependencyCheck/wiki/Suppression |
| CLI reference | https://github.com/dependency-check/DependencyCheck/wiki/CLI |
| Maven plugin | https://github.com/dependency-check/DependencyCheck/tree/main/maven |
