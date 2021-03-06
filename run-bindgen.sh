#!/bin/sh

$HOME/.cargo/bin/bindgen odpi/include/dpi.h -o src/binding.rs \
  --distrust-clang-mangling \
  --whitelist-type "^dpi.*" \
  --whitelist-function "^dpi.*" \
  --whitelist-var "^DPI_.*" \
  --constified-enum dpiAuthMode \
  --constified-enum dpiConnCloseMode \
  --constified-enum dpiCreateMode \
  --constified-enum dpiDeqMode \
  --constified-enum dpiDeqNavigation \
  --constified-enum dpiEventType \
  --bitfield-enum   dpiExecMode \
  --bitfield-enum   dpiFetchMode \
  --constified-enum dpiMessageDeliveryMode \
  --constified-enum dpiMessageState \
  --constified-enum dpiNativeTypeNum \
  --bitfield-enum   dpiOpCode \
  --constified-enum dpiOracleTypeNum \
  --constified-enum dpiPoolCloseMode \
  --constified-enum dpiPoolGetMode \
  --constified-enum dpiPurity \
  --constified-enum dpiShutdownMode \
  --constified-enum dpiStartupMode \
  --constified-enum dpiStatementType \
  --constified-enum dpiSubscrNamespace \
  --constified-enum dpiSubscrProtocol \
  --bitfield-enum   dpiSubscrQOS \
  --constified-enum dpiVisibility \
  --no-prepend-enum-name \
  -- -Iodpi/include
