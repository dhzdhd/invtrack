package dev.dhzdhd.invtrack.settings.views

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import dev.dhzdhd.invtrack.settings.models.Settings
import dev.dhzdhd.invtrack.settings.models.Theme
import dev.dhzdhd.invtrack.settings.viewmodels.SettingsAction
import invtrack.composeapp.generated.resources.Res
import invtrack.composeapp.generated.resources.back_arrow
import kotlinx.serialization.Serializable
import org.jetbrains.compose.resources.painterResource

@Serializable
object SettingsRoute

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsView(
    navController: NavHostController,
    settings: Settings,
    dispatch: (SettingsAction) -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Settings") }, navigationIcon = {
                IconButton(onClick = {
                    navController.navigateUp()
                }) {
                    Icon(
                        painter = painterResource(Res.drawable.back_arrow),
                        contentDescription = "Back"
                    )
                }
            })
        },
        content = { paddingValues ->
            Column(modifier = Modifier.padding(paddingValues)) {
                ListItem(
                    modifier = Modifier.clickable(true, onClick = {}),
                    headlineContent = { Text("Theme", style = MaterialTheme.typography.titleLarge) },
                    trailingContent = {
                        SingleChoiceSegmentedButtonRow {
                            Theme.entries.toTypedArray().map { theme ->
                                SegmentedButton(
                                    selected = settings.theme == theme,
                                    modifier = Modifier.padding(2.dp),
                                    onClick = { dispatch(SettingsAction.UpdateTheme(theme)) },
                                    shape = RoundedCornerShape(10.dp)
                                ) {
                                    Text(theme.name)
                                }
                            }
                        }
                    })
            }
        }
    )
}
