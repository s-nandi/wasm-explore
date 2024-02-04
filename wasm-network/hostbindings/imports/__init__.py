from .network_provider import HostNetworkProvider
from dataclasses import dataclass

@dataclass
class RootImports:
    network_provider: HostNetworkProvider
