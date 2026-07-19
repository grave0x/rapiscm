# PyMISP — Python Library for MISP

Official Python library for MISP threat intelligence platform API. Event creation, attribute management, feed sync, and Galaxy operations.

## How It Works

PyMISP wraps the MISP REST API (JSON over HTTP). Provides both high-level object-oriented and low-level dict-based interfaces. Handles authentication, pagination, error handling, and file uploads.

**Key classes:**

| Class | Purpose |
|-------|---------|
| `PyMISP` | Core API client. Event CRUD, search, feed sync, admin operations |
| `MISPEvent` | Event builder with validation, attribute/object helpers |
| `MISPAttribute` | Individual indicator with type, value, distribution, comment |
| `MISPObject` | Complex indicator (file with name/size/hash, email with attachments) |
| `MISPGalaxy` | Galaxy/Cluster references (MITRE ATT&CK, threat actors) |
| `MISPFeed` | Feed configuration management |

## Manual

```python
from pymisp import PyMISP, MISPEvent, MISPObject
from pymisp.tools import FileObject  # specialized objects

# Connect
misp = PyMISP('https://misp.local', 'API_KEY', ssl=False)

# Create event
event = MISPEvent()
event.info = 'Suspicious domain observed in phishing campaign'
event.distribution = 1  # 0=org, 1=community, 2=connected, 3=all
event.threat_level_id = 2  # 1=high, 2=medium, 3=low
event.analysis = 0  # 1=initial, 1=ongoing, 2=complete
misp.add_event(event)

# Add attributes
event.add_attribute('ip-src', '1.2.3.4')
event.add_attribute('domain', 'evil.example.com')
event.add_attribute('url', 'https://evil.example.com/phish')
event.add_attribute('md5', 'd41d8cd98f00b204e9800998ecf8427e')
misp.update_event(event)

# Add file object
file_obj = FileObject('malware.exe')
file_obj.add_attribute('md5', 'd41d8cd98f00b204e9800998ecf8427e')
file_obj.add_attribute('size', '10240')
event.add_object(file_obj)
misp.update_event(event)

# Add Galaxy (MITRE ATT&CK)
misp.add_galaxy_cluster_reference(event, 'mitre-attack-pattern', 'T1566.001')

# Search events
results = misp.search('events', value='1.2.3.4')
results = misp.search_index('events', tags='tlp:green')

# Search attributes
attrs = misp.search('attributes', value='evil.example.com')

# Export IoCs as CSV
iocs = misp.search('attributes', type='ip-src', publish_timestamp='30d')
```

### Feed Operations

```python
# Get feeds
feeds = misp.feeds()
for feed in feeds:
    if feed['Feed']['enabled']:
        print(feed['Feed']['name'], feed['Feed']['url'])

# Enable feed
misp.enable_feed(feed_id=5)

# Fetch feed
misp.fetch_feed(feed_id=5)
```

### Batch Processing

```python
# Download and process all IoCs from last 24h
import datetime

yesterday = datetime.datetime.now() - datetime.timedelta(days=1)
events = misp.search_index('events', publish_timestamp=yesterday.strftime('%d'))

for event_summary in events:
    event = misp.get_event(event_summary['Event']['id'])
    for attr in event['Event']['Attribute']:
        print(f"{attr['type']}: {attr['value']}")
```

## Install

```bash
pip install pymisp
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/MISP/PyMISP |
| PyPI | https://pypi.org/project/pymisp/ |
| Docs | https://pymisp.readthedocs.io/ |
| MISP objects | https://github.com/MISP/misp-objects |
| MISP galaxy | https://github.com/MISP/misp-galaxy |
