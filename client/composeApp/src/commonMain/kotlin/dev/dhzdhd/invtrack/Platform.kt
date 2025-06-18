package dev.dhzdhd.invtrack

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform