package dev.dhzdhd.invtrack.settings.viewmodels

import androidx.lifecycle.ViewModel
import dev.dhzdhd.invtrack.settings.models.Settings
import dev.dhzdhd.invtrack.settings.models.Theme
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.koin.android.annotation.KoinViewModel

@KoinViewModel
class SettingsViewModel : ViewModel() {
    private val state = MutableStateFlow(Settings.Default)
    val settings = state.asStateFlow()

    fun updateTheme(theme: Theme) {
        state.value = state.value.copy(theme = theme)
    }
}
