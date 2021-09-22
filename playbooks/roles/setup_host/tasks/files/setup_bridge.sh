#!/bin/bash

BRIDGE=fcbridge
INTERFACE=$(ip route get 8.8.8.8 | awk '{ print $5; exit }')
IP=$(hostname -I)
IP_WITH_CIDR=$(ip addr show dev $INTERFACE | grep "inet " | awk '{print $2}')
ROUTE=$(ip route list dev $INTERFACE | grep -v default | cut -f 1 -d ' ')
DEFAULT_ROUTE=$(ip route list dev $INTERFACE | grep default | awk '{print $3}')

ovs-vsctl add-br $BRIDGE
ovs-vsctl add-port $BRIDGE $INTERFACE
ip addr flush dev $INTERFACE
ip addr add $IP_WITH_CIDR brd + dev $BRIDGE
ip link set $BRIDGE up
ip route add default via $DEFAULT_ROUTE
ip route add $ROUTE dev $BRIDGE proto kernel scope link src $IP

exit 0
