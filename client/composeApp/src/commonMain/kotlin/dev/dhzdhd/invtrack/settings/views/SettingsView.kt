package dev.dhzdhd.invtrack.settings.views

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.RectangleShape
import dev.dhzdhd.invtrack.settings.models.Theme
import dev.dhzdhd.invtrack.settings.viewmodels.SettingsViewModel
import org.koin.compose.viewmodel.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsView() {
    val settingsViewModel = koinViewModel<SettingsViewModel>()
    val settings by settingsViewModel.settings.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Settings") })
        },
        content = { paddingValues ->
            Column(modifier = Modifier.padding(paddingValues)) {
                SingleChoiceSegmentedButtonRow {
                    Theme.entries.toTypedArray().map { theme ->
                        SegmentedButton(
                            selected = settings.theme == theme,
                            onClick = { settingsViewModel.updateTheme(theme) },
                            shape = RectangleShape
                        ) {
                            Text(theme.name)
                        }
                    }
                }
            }
        }
    )
}