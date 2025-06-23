package dev.dhzdhd.invtrack

import org.koin.core.annotation.ComponentScan
import org.koin.core.annotation.Module
import org.koin.core.annotation.Single
import org.koin.core.context.startKoin
import org.koin.core.parameter.parametersOf
import org.koin.dsl.KoinAppDeclaration
import org.koin.ksp.generated.*
import org.koin.mp.KoinPlatform

@Module
@ComponentScan("dev.dhzdhd.invtrack.settings")
class SettingsModule

@Module(includes = [SettingsModule::class])
class AppModule

fun initKoin(config : KoinAppDeclaration ?= null) {
    startKoin {
        modules(
            AppModule().module,
        )
        config?.invoke(this)
    }
}