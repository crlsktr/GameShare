import { Injectable } from '@angular/core';
import { HttpService } from './http.service';

@Injectable({
	providedIn: 'root',
})
export class UserService {
	constructor(private httpService: HttpService) {}

	public checkUsedUsername(userName: String): Promise<Boolean> {
		return this.httpService.post(
			'/user/checkUserName',
			{ username: userName, pass_phrase: '' },
			{ withCredentials: true }
		);
	}
}
