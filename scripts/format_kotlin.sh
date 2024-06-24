#!/bin/bash
ROMER_ANDROID_DIR="bindings/kotlin/romer-android"
ROMER_JVM_DIR="bindings/kotlin/romer-jvm"

# Run ktlintFormat in romer-android
(
  cd $ROMER_ANDROID_DIR || exit 1
  ./gradlew ktlintFormat
)

# Run ktlintFormat in romer-jvm
(
  cd $ROMER_JVM_DIR || exit 1
  ./gradlew ktlintFormat
)
