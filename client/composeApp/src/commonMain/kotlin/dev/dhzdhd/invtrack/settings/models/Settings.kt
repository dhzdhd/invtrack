package dev.dhzdhd.invtrack.settings.models

import kotlinx.serialization.Serializable

enum class Theme {
    LIGHT, DARK
}

@Serializable
data class Settings(val theme: Theme) {
    companion object {
        val Default = Settings(Theme.DARK)
    }
}
