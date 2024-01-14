#!/usr/bin/env bash

codesign --entitlements etc/pkginstaller/macos/entitlements.xml -s Jardin-MachO target/release/cmdb-agent
