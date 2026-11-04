#ifndef RUNNER_WORKSPACE_CHANNEL_H_
#define RUNNER_WORKSPACE_CHANNEL_H_

#include <flutter/binary_messenger.h>
#include <windows.h>

// Native folder picker and Explorer reveal for com.openmuse.host/workspace.
// The macOS host implements the same channel in MainFlutterWindow.swift.
void RegisterWorkspaceChannel(flutter::BinaryMessenger* messenger, HWND owner);
void UnregisterWorkspaceChannel();

#endif  // RUNNER_WORKSPACE_CHANNEL_H_
