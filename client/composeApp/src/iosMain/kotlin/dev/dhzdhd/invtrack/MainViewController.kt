package dev.dhzdhd.invtrack

import androidx.compose.ui.window.ComposeUIViewController

fun MainViewController() = ComposeUIViewController {
    initKoin()
    App()
}