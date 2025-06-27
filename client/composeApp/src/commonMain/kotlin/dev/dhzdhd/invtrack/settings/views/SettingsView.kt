package dev.dhzdhd.invtrack.settings.views

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.RectangleShape
import dev.dhzdhd.invtrack.settings.models.Settings
import dev.dhzdhd.invtrack.settings.models.Theme
import dev.dhzdhd.invtrack.settings.viewmodels.SettingsAction
import kotlinx.serialization.Serializable
import org.jetbrains.compose.ui.tooling.preview.Preview

@Serializable
object SettingsRoute

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsView(
    settings: Settings,
    dispatch: (SettingsAction) -> Unit,
) {
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
                            onClick = { dispatch(SettingsAction.UpdateTheme(theme)) },
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

@Preview
@Composable
fun SettingsViewPreview() {
    SettingsView(
        settings = Settings.Default,
        dispatch = {}
    )
}