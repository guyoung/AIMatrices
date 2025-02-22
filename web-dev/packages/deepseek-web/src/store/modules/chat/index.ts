import { defineStore } from 'pinia'

import { router } from '@/router'

import { get, post } from '@/utils/request'


export const useChatStore = defineStore('chat-store', {
    state: (): Chat.ChatState => ({
        usingContext: true,
        conversations: [],
        active: null,
        messages: []
    }),

    getters: {
    },

    actions: {
        setUsingContext(context: boolean) {
            this.usingContext = context
        },

        async addConversation(conversation: Partial<Chat.ChatConversation>) {
            let data: Chat.ChatConversation = {
                id: null,
                title: conversation.title,
                isEdit: conversation.isEdit,
                seq_num: conversation.seq_num,
                created_at: null,
                updated_at: null,
                user_id: null
            }

            try {
                let res = await post({
                    url: '/chat/add-conversation',
                    data: data
                })

                let result = res.result

                if (result.length > 0) {
                    await this.fetchConversations()

                    return result[0]
                }               

            } catch (e) {
                console.log(e)
            }

            return null

        },

        async updateConversation(conversation: Chat.ChatConversation) {
            try {
                await post({
                    url: '/chat/update-conversation',
                    data: conversation
                });

                await this.fetchConversations()

            } catch (e) {
                console.log(e)
            }
        },


        async deleteConversation(conversation: Chat.ChatConversation) {
            try {
                await post({
                    url: '/chat/delete-conversation',
                    data: conversation
                });

                await this.fetchConversations();

            } catch (e) {
                console.log(e)
            }
        },

        async deleteConversations() {
            try {
                await post({
                    url: '/chat/delete-conversations'
                });

                await this.fetchConversations();

            } catch (e) {
                console.log(e)
            }
        },

        async addMessage(conversation_id: string, message: Partial<Chat.ChatMessage>) {
            let data: Chat.ChatMessage = {
                id: null,
                content: message.content,
                role: message.role,
                status: 1,
                seq_num: message.seq_num,
                created_at: null,
                updated_at: null,
                conversation_id: conversation_id,
                user_id: null,
                parent_id: message.parent_id,
                request_id: message.request_id,
                metadata: {},

                inversion: message.inversion,
                error: message.error,
                loading: message.loading,
            }

            try {
                let res = await post({
                    url: '/chat/add-message',
                    data: data
                });

                let result = res.result
                if (result.length > 0) {
                    await this.fetchMessages();

                    return result[0]
                }                

            } catch (e) {
                console.log(e)
            }

            return null

        },

        async updateMessage(message: Chat.ChatMessage) {
            try {
                await post({
                    url: '/chat/update-message',
                    data: message
                });

                await this.fetchMessages();

            } catch (e) {
                console.log(e)
            }
        },

        async batchUpdateMessage(conversation_id: string, loading: boolean) {
            try {
                await post({
                     url: `/chat/batch-update-messages?conversation=${conversation_id}&loading=${loading}`
             
                });

                await this.fetchMessages();

            } catch (e) {
                console.log(e)
            }
        },



        async deleteMessage(message: Chat.ChatMessage) {
            try {
                await post({
                    url: '/chat/delete-message',
                    data: message
                });

                await this.fetchMessages();

            } catch (e) {
                console.log(e)
            }
        },

        
        async deleteMessages(conversation_id: string) {
            try {
                await post({
                    url: `/chat/delete-messages?conversation=${conversation_id}`,                   
                });

                await this.fetchMessages();

            } catch (e) {
                console.log(e)
            }
        },


        async fetchConversations() {
            try {
                const res = await get({
                    url: '/chat/conversations'
                });

                this.conversations = res.result;

            } catch (e) {
                console.log(e)
            }

            return [];
        },

        async fetchMessages() {
            try {
                const res = await get({
                    url: `/chat/messages?conversation=${this.active}`
                });

                this.messages = res.result;

            } catch (e) {
                console.log(e)
            }

            return [];
        },

        async setActiveConversation(id: string) {  
            this.active = id

            await this.fetchMessages()

            return await this.reloadRoute(id)
        },

        async reloadRoute(conversationID?: string) {      
            await router.push({ name: 'Chat', params: { conversationID } })
        },

    }
})

