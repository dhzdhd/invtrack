package dev.dhzdhd.invtrack.home.models

import kotlin.time.Duration

data class Item(
    val id: String,
    val name: String,
    val quantity: Int,
    val duration: Duration,
    val description: String? = null,
    val imageUrl: String? = null
)
