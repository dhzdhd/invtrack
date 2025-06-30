package dev.dhzdhd.invtrack.home.repositories

import dev.dhzdhd.invtrack.home.models.Item
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import org.koin.core.annotation.Single

@Single
class HomeRepository(private val httpClient: HttpClient) {
    suspend fun getItems(): List<Item> {
        httpClient.get {
            url("https://api.example.com/items")
        }.body<List<Item>>().also {
            println(it)
        }
        return listOf(
            Item(id = "1", name = "Item 1", quantity = 10, duration = kotlin.time.Duration.ZERO),
            Item(id = "2", name = "Item 2", quantity = 5, duration = kotlin.time.Duration.ZERO)
        )
    }
}