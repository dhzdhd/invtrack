package dev.dhzdhd.invtrack.home.viewmodels

import androidx.lifecycle.ViewModel
import dev.dhzdhd.invtrack.home.models.Item
import dev.dhzdhd.invtrack.home.repositories.HomeRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.koin.android.annotation.KoinViewModel

sealed class HomeAction {
    data class AddItem(val item: Item) : HomeAction()
}

@KoinViewModel
class HomeViewModel(private val repository: HomeRepository) : ViewModel() {
    private val state = MutableStateFlow<List<Item>>(listOf())
    val items = state.asStateFlow()

    fun dispatch(action: HomeAction) {
        when (action) {
            is HomeAction.AddItem -> addItem(action.item)
        }
    }

    private fun getItems(): List<Item> {
//        viewModelScope {
//            repository.getItems()
//        }
        return state.value
    }

    private fun addItem(item: Item) {
        println("Item added: $item")
        state.value = state.value + item
    }
}