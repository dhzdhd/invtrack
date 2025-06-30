package dev.dhzdhd.invtrack.home.views

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import dev.dhzdhd.invtrack.home.models.Item
import dev.dhzdhd.invtrack.home.viewmodels.HomeAction
import dev.dhzdhd.invtrack.settings.views.SettingsRoute
import invtrack.composeapp.generated.resources.Res
import invtrack.composeapp.generated.resources.settings
import kotlinx.serialization.Serializable
import org.jetbrains.compose.resources.painterResource

@Serializable
object HomeRoute

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeView(
    navController: NavHostController,
    items: List<Item>,
    dispatch: (HomeAction) -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Home") }, actions = {
                IconButton(onClick = {
                    navController.navigate(SettingsRoute)
                }) {
                    Icon(
                        painter = painterResource(Res.drawable.settings),
                        contentDescription = "Settings"
                    )
                }
            })
        },
        content = { paddingValues ->
            Column(modifier = Modifier.padding(paddingValues)) {
                if (items.isEmpty()) {
                    Text("No items yet")
                } else {
                    items.map { it ->
                        ListItem(
                            modifier = Modifier.clickable(true, onClick = {}),
                            headlineContent = { Text(it.name, style = MaterialTheme.typography.titleLarge) },
                        )
                    }
                }
            }
        }
    )
}
