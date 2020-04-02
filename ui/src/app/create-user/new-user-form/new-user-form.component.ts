import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';

@Component({
	selector: 'app-new-user-form',
	templateUrl: './new-user-form.component.html',
	styleUrls: ['./new-user-form.component.scss'],
})
export class NewUserFormComponent implements OnInit {
	newUserForm: FormGroup;
	userNameControl: FormControl = new FormControl('');
	passphraseControl: FormControl = new FormControl('');
	default = 'hello';

	constructor() {
		this.newUserForm = new FormGroup({
			userName: this.userNameControl,
			passPhrase: this.passphraseControl,
		});
	}

	ngOnInit(): void {}
}
