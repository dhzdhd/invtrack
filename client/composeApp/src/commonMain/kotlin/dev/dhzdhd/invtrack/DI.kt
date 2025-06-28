package dev.dhzdhd.invtrack

import io.ktor.client.*
import org.koin.core.annotation.ComponentScan
import org.koin.core.annotation.Module
import org.koin.core.context.startKoin
import org.koin.dsl.KoinAppDeclaration
import org.koin.ksp.generated.module


@Module
@ComponentScan(
    "dev.dhzdhd.invtrack.settings.viewmodels",
    "dev.dhzdhd.invtrack.settings.repositories",
    "dev.dhzdhd.invtrack.settings.models",
    "dev.dhzdhd.invtrack.settings.views"
)
class SettingsModule

@Module
@ComponentScan(
    "dev.dhzdhd.invtrack.home.viewmodels",
    "dev.dhzdhd.invtrack.home.repositories",
    "dev.dhzdhd.invtrack.home.models",
    "dev.dhzdhd.invtrack.home.views"
)
class HomeModule {
//    @Single
//    fun httpClient(json: Json) = HttpClient {
//        install(ContentNegotiation) {
//            json(json, contentType = ContentType.Any)
//        }
//    }
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