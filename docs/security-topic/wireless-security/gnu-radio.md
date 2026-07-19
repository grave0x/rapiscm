# GNU Radio — Software-Defined Radio Framework

Open-source SDR development toolkit. Signal processing blocks for building radio applications — AM/FM, digital modes, spectrum analysis, custom modulation.

## How It Works

GNU Radio Companion (GRC) — visual flowgraph editor. Drag-and-drop signal processing blocks connected by data streams. Underlying C++/Python runtime for real-time DSP.

**Architecture:**
- **Blocks** — Sources (RTL-SDR, HackRF, USRP, file), sinks (audio, file, network, GUI), processing (filters, FFT, decoders, modulators)
- **Flowgraphs** — Directed graph of connected blocks. Each block runs in own thread (thread-per-block scheduler).
- **Volk** — Vector-Optimized Library of Kernels. SIMD-accelerated math.
- **OOT modules** — Out-of-tree. Community extensions for specific protocols.

**Common flowgraphs:** FM radio receiver, ADS-B decoder, APT weather satellite decoder, GSM baseband analysis, spectrum waterfall display.

## Manual

```bash
# Launch GNU Radio Companion GUI
gnuradio-companion

# Run flowgraph from command line
python3 my_flowgraph.py

# Run gr-osmosdr examples
gr_osmocom_fft -f 433e6 -s 2e6

# FM receiver example
python3 /usr/share/gnuradio/examples/audio/fm_demod.py -f 100.7e6
```

## Build

```bash
git clone https://github.com/gnuradio/gnuradio.git
cd gnuradio
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install gnuradio gnuradio-dev gr-osmosdr

# macOS
brew install gnuradio

# Windows
# Download installer from gnuradio.org

# Docker
docker pull gnuradio/gnuradio:latest

# PyBOMBS (build manager)
pip install pybombs
pybombs prefix init ~/gnuradio
pybombs install gnuradio
```

## Package

| Manager | Command |
|---------|---------|
| apt | `sudo apt install gnuradio` |
| Homebrew | `brew install gnuradio` |
| Conda | `conda install -c conda-forge gnuradio` |
| Docker | `docker pull gnuradio/gnuradio` |
| Snap | `snap install gnuradio` |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.gnuradio.org/ |
| GitHub | https://github.com/gnuradio/gnuradio |
| Docs | https://wiki.gnuradio.org/ |
| GRC tutorials | https://wiki.gnuradio.org/index.php?title=Guided_Tutorials |
| OOT modules | https://wiki.gnuradio.org/index.php/OutOfTreeModules |
| GNU Radio Conference | https://gnuradio.org/grcon/ |
| CGRAN (repos) | https://www.cgran.org/ |
