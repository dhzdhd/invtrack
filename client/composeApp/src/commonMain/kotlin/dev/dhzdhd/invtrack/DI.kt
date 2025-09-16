package dev.dhzdhd.invtrack

import io.ktor.client.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import org.koin.core.annotation.ComponentScan
import org.koin.core.annotation.Module
import org.koin.core.annotation.Single
import org.koin.core.context.startKoin
import org.koin.dsl.KoinAppDeclaration
import org.koin.ksp.generated.module


@Module
@ComponentScan("dev.dhzdhd.invtrack.settings")
class SettingsModule

@Module
@ComponentScan("dev.dhzdhd.invtrack.home")
class HomeModule {
    @Single
    fun httpClient() = HttpClient {
        install(ContentNegotiation) {
            json()
        }
    }
}

@Module(includes = [SettingsModule::class, HomeModule::class])
class AppModule

fun initKoin(config: KoinAppDeclaration? = null) {
    startKoin {
        modules(
            AppModule().module,
        )
        config?.invoke(this)
    }
}