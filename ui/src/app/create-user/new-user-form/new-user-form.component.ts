import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { UserService } from 'src/services/user.service';

@Component({
	selector: 'app-new-user-form',
	templateUrl: './new-user-form.component.html',
	styleUrls: ['./new-user-form.component.scss'],
})
export class NewUserFormComponent implements OnInit {
	newUserForm: FormGroup;
	userNameControl: FormControl = new FormControl('');
	passphraseControl: FormControl = new FormControl('');

	used: Boolean;

	constructor(private userService: UserService) {
		this.newUserForm = new FormGroup({
			userName: this.userNameControl,
			passPhrase: this.passphraseControl,
		});
	}

	ngOnInit(): void {}

	verifyUser(event) {
		this.userService
			.checkUsedUsername(this.userNameControl.value)
			.then(x => (this.used = x));
	}
}
