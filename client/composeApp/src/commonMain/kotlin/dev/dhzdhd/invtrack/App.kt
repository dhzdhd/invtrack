package dev.dhzdhd.invtrack

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import dev.dhzdhd.invtrack.home.viewmodels.HomeViewModel
import dev.dhzdhd.invtrack.home.views.HomeRoute
import dev.dhzdhd.invtrack.home.views.HomeView
import dev.dhzdhd.invtrack.settings.models.Theme
import dev.dhzdhd.invtrack.settings.viewmodels.SettingsViewModel
import dev.dhzdhd.invtrack.settings.views.SettingsRoute
import dev.dhzdhd.invtrack.settings.views.SettingsView
import org.jetbrains.compose.ui.tooling.preview.Preview
import org.koin.compose.viewmodel.koinViewModel

@Composable
@Preview
fun App() {
    val navController = rememberNavController()
    val settingsViewModel = koinViewModel<SettingsViewModel>()
    val settings by settingsViewModel.settings.collectAsState()

    MaterialTheme(
        colorScheme = when (settings.theme) {
            Theme.LIGHT -> lightColorScheme()
            Theme.DARK -> darkColorScheme()
        }
    ) {
        NavHost(navController = navController, startDestination = HomeRoute) {
            composable<HomeRoute> {
                val homeViewModel = koinViewModel<HomeViewModel>()
                val items by homeViewModel.items.collectAsState()
                HomeView(navController, items, homeViewModel::dispatch)
            }
            composable<SettingsRoute> {
                SettingsView(navController, settings, settingsViewModel::dispatch)
            }
        }
    }
}