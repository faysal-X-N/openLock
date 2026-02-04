# Shield Android App Guide

This directory contains the Android client source code for the Shield Password Manager.

## Prerequisites
- Android Studio Iguana or newer
- JDK 17+
- Android SDK API 34

## Project Structure
- `app/src/main/java/com/shield/android/ui`: UI components (Activities, ViewModels)
- `app/src/main/res/layout`: XML Layouts
- `app/build.gradle`: App-level dependency configuration

## Features Implemented
1. **Material Design UI**: Uses standard Material components and layouts.
2. **Data Visualization**: Integrated `MPAndroidChart` for statistical charts.
   - Bar Chart: Password Strength Distribution
   - Pie Chart: Account Type Distribution
3. **Architecture**: MVVM pattern with `ChartViewModel`.
4. **Responsiveness**: `ConstraintLayout` and `ScrollView` for screen adaptation.

## How to Run
1. Open **Android Studio**.
2. Select **Open an existing Android Studio project**.
3. Navigate to the `shield-android` directory created in this workspace.
4. Wait for Gradle sync to complete.
5. Connect an Android device or start an Emulator (API 24+).
6. Click **Run > Run 'app'**.

## Note on Compilation
This code was generated in a remote environment without the Android SDK. While the code structure and syntax are correct, you may need to resolve minor dependency version conflicts or local SDK path issues (`local.properties`) when opening in Android Studio for the first time.
