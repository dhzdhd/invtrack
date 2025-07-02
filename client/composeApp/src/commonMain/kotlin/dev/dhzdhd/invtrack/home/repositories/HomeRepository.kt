package dev.dhzdhd.invtrack.home.repositories

import dev.dhzdhd.invtrack.home.models.Item
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import org.koin.core.annotation.Single

@Single
class HomeRepository(private val httpClient: HttpClient) {
    suspend fun getItems(): Result<List<Item>> {
        return runCatching {
            httpClient.get {
                url("http://localhost:8080/items")
            }.body<List<Item>>().also {
                println(it)
            }

            return Result.success(listOf())
        }
    }
}