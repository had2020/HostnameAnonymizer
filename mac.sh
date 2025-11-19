#!/bin/bash

for interface in $(ls /sys/class/net/ | grep -v lo); do
    echo "Processing interface: $interface"
    
    sudo ip link set dev "$interface" down
    
    sudo macchanger -r "$interface"
    
    sudo ip link set dev "$interface" up
    
    echo "Finished processing $interface."
done

