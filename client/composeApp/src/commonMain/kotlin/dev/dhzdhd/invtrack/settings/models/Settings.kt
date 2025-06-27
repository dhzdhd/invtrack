package dev.dhzdhd.invtrack.settings.models

import kotlinx.serialization.Serializable

enum class Theme {
    LIGHT, DARK, SYSTEM
}

@Serializable
data class Settings(val theme: Theme) {
    companion object {
        val Default = Settings(Theme.SYSTEM)
    }
}
