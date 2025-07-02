package dev.dhzdhd.invtrack.home.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlin.time.Duration


@Serializable
data class Item(
    val id: String,
    val name: String,
    val description: String,
    val quantity: Int,
    val categoryId: String,
    val price: Int,
    val imageUrl: String,
    val duration: Duration,
    val expiryDate: Instant,
    val createdAt: Instant,
    val updatedAt: Instant,
)
