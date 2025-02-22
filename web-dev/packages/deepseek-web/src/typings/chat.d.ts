declare namespace Chat {

	/***
	interface Chat {
		dateTime: string
		text: string,
		role: String,
		inversion?: boolean
		error?: boolean
		loading?: boolean
		conversationOptions?: ConversationRequest | null
		requestOptions: { prompt: string; options?: ConversationRequest | null }
	}

	interface History {
		title: string
		isEdit: boolean
		uuid: number
	}


	interface ChatState {
		active: number | null
		usingContext: boolean;
		history: History[]
		chat: { uuid: number; data: Chat[] }[]
	}
		
	interface ConversationRequest {
		conversationId?: string
		parentMessageId?: string
	}

	***/



	interface ChatState {		
		usingContext: boolean,
		conversations: ChatConversation[],  
		active: string | null,		
		messages: ChatMessage[]	
	}


	interface ChatConversation {
		id: string | null,	
		title: string,
		isEdit: boolean,
		seq_num: number,
		created_at:  Date | null,	
		updated_at:  Date | null,	
		user_id: string | null,	
	}

	interface ChatMessage {
		id: string | null,	
		content: string,
		role: String,
		status: number,
		seq_num: number,
		created_at:  Date | null,	
		updated_at:  Date | null,	
		conversation_id: string,
		user_id: string | null,	
		parent_id: string | null,	
		request_id: string | null,	
		metadata: any,
		
		inversion?: boolean,
		error?: boolean,
		loading?: boolean,
	}


	interface ChatResponse {
		conversationId: string
		detail: {
			choices: { finish_reason: string; index: number; logprobs: any; text: string }[]
			created: number
			id: string
			model: string
			object: string
			usage: { completion_tokens: number; prompt_tokens: number; total_tokens: number }
		}
		id: string
		parentMessageId: string
		role: string
		text: string
	}
}
